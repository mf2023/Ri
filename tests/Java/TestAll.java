// Copyright 2025-2026 Wenze Wei. All Rights Reserved.
//
// Test all DMSC Java bindings

import com.dunimd.dmsc.*;
import com.dunimd.dmsc.cache.*;
import com.dunimd.dmsc.validation.*;
import com.dunimd.dmsc.auth.*;
import com.dunimd.dmsc.gateway.*;
import com.dunimd.dmsc.queue.*;
import com.dunimd.dmsc.database.*;
import com.dunimd.dmsc.servicemesh.*;
import com.dunimd.dmsc.observability.*;
import com.dunimd.dmsc.device.*;
import com.dunimd.dmsc.log.*;
import com.dunimd.dmsc.fs.*;
import com.dunimd.dmsc.grpc.*;
import com.dunimd.dmsc.ws.*;
import com.dunimd.dmsc.protocol.*;
import com.dunimd.dmsc.hooks.*;
import com.dunimd.dmsc.modulerpc.*;

/**
 * Comprehensive test suite for all DMSC Java bindings.
 *
 * This test file is located in the unified tests directory (tests/Java/)
 * rather than in the source code directory, following the project's testing convention.
 */
public class TestAll {
    public static void main(String[] args) {
        System.out.println("=== DMSC Java Binding Test ===\n");
        
        int passed = 0;
        int failed = 0;
        
        // Test DMSCAppBuilder
        try {
            DMSCAppBuilder builder = new DMSCAppBuilder();
            System.out.println("[PASS] DMSCAppBuilder created");
            builder.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCAppBuilder: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCAppBuilder chaining (returns new instance)
        try {
            DMSCAppBuilder builder1 = new DMSCAppBuilder();
            DMSCAppBuilder builder2 = builder1.withConfig("config.yaml");
            
            if (builder1 != builder2) {
                System.out.println("[PASS] DMSCAppBuilder chaining creates new instance");
                passed++;
            } else {
                System.out.println("[FAIL] DMSCAppBuilder chaining should create new instance");
                failed++;
            }
            
            builder1.close();
            builder2.close();
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCAppBuilder chaining: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCConfig
        try {
            DMSCConfig config = new DMSCConfig();
            System.out.println("[PASS] DMSCConfig created");
            config.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCConfig: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCCacheModule
        try {
            DMSCCacheConfig cacheConfig = new DMSCCacheConfig();
            DMSCCacheModule cache = new DMSCCacheModule(cacheConfig);
            System.out.println("[PASS] DMSCCacheModule created");
            cache.close();
            cacheConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCCacheModule: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCValidationModule
        try {
            DMSCValidationResult result = DMSCValidationModule.validateEmail("test@example.com");
            System.out.println("[PASS] DMSCValidationModule.validateEmail: valid=" + result.isValid());
            result.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCValidationModule: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCAuthModule
        try {
            DMSCAuthConfig authConfig = new DMSCAuthConfig();
            DMSCAuthModule auth = new DMSCAuthModule(authConfig);
            System.out.println("[PASS] DMSCAuthModule created");
            auth.close();
            authConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCAuthModule: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCGateway
        try {
            DMSCGateway gateway = new DMSCGateway();
            System.out.println("[PASS] DMSCGateway created");
            gateway.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCGateway: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCQueueModule
        try {
            DMSCQueueConfig queueConfig = new DMSCQueueConfig();
            DMSCQueueModule queue = new DMSCQueueModule(queueConfig);
            System.out.println("[PASS] DMSCQueueModule created");
            queue.close();
            queueConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCQueueModule: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCDatabaseConfig
        try {
            DMSCDatabaseConfig dbConfig = new DMSCDatabaseConfig();
            System.out.println("[PASS] DMSCDatabaseConfig created");
            dbConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCDatabaseConfig: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCServiceMesh
        try {
            DMSCServiceMeshConfig meshConfig = new DMSCServiceMeshConfig();
            DMSCServiceMesh mesh = new DMSCServiceMesh(meshConfig);
            System.out.println("[PASS] DMSCServiceMesh created");
            mesh.close();
            meshConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCServiceMesh: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCObservabilityModule
        try {
            DMSCObservabilityConfig obsConfig = new DMSCObservabilityConfig();
            DMSCObservabilityModule obs = new DMSCObservabilityModule(obsConfig);
            System.out.println("[PASS] DMSCObservabilityModule created");
            obs.close();
            obsConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCObservabilityModule: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCDeviceControlModule
        try {
            DMSCDeviceControlConfig deviceConfig = new DMSCDeviceControlConfig();
            DMSCDeviceControlModule device = new DMSCDeviceControlModule(deviceConfig);
            System.out.println("[PASS] DMSCDeviceControlModule created");
            device.close();
            deviceConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCDeviceControlModule: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCLogger
        try {
            DMSCLogger logger = new DMSCLogger();
            System.out.println("[PASS] DMSCLogger created");
            logger.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCLogger: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCFileSystem
        try {
            DMSCFileSystem fs = new DMSCFileSystem();
            System.out.println("[PASS] DMSCFileSystem created");
            fs.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCFileSystem: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCGrpcServer
        try {
            DMSCGrpcServer grpcServer = new DMSCGrpcServer();
            System.out.println("[PASS] DMSCGrpcServer created");
            grpcServer.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCGrpcServer: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCGrpcClient
        try {
            DMSCGrpcClient grpcClient = new DMSCGrpcClient();
            System.out.println("[PASS] DMSCGrpcClient created");
            grpcClient.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCGrpcClient: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCWSServer
        try {
            DMSCWSServer wsServer = new DMSCWSServer();
            System.out.println("[PASS] DMSCWSServer created");
            wsServer.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCWSServer: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCWSClient
        try {
            DMSCWSClient wsClient = new DMSCWSClient();
            System.out.println("[PASS] DMSCWSClient created");
            wsClient.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCWSClient: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCProtocolManager
        try {
            DMSCProtocolManager protocol = new DMSCProtocolManager();
            System.out.println("[PASS] DMSCProtocolManager created");
            protocol.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCProtocolManager: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCHookBus
        try {
            DMSCHookBus hookBus = new DMSCHookBus();
            System.out.println("[PASS] DMSCHookBus created");
            hookBus.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCHookBus: " + e.getMessage());
            failed++;
        }
        
        // Test DMSCModuleRPC
        try {
            DMSCModuleRPC moduleRpc = new DMSCModuleRPC();
            System.out.println("[PASS] DMSCModuleRPC created");
            moduleRpc.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] DMSCModuleRPC: " + e.getMessage());
            failed++;
        }
        
        System.out.println("\n=== Test Summary ===");
        System.out.println("Passed: " + passed);
        System.out.println("Failed: " + failed);
        System.out.println("Total:  " + (passed + failed));
        
        if (failed > 0) {
            System.exit(1);
        }
    }
}
