// Copyright 2025-2026 Wenze Wei. All Rights Reserved.
//
// Test all Ri Java bindings

import com.dunimd.ri.*;
import com.dunimd.ri.cache.*;
import com.dunimd.ri.validation.*;
import com.dunimd.ri.auth.*;
import com.dunimd.ri.gateway.*;
import com.dunimd.ri.queue.*;
import com.dunimd.ri.database.*;
import com.dunimd.ri.servicemesh.*;
import com.dunimd.ri.observability.*;
import com.dunimd.ri.device.*;
import com.dunimd.ri.log.*;
import com.dunimd.ri.fs.*;
import com.dunimd.ri.grpc.*;
import com.dunimd.ri.ws.*;
import com.dunimd.ri.protocol.*;
import com.dunimd.ri.hooks.*;
import com.dunimd.ri.modulerpc.*;

/**
 * Comprehensive test suite for all Ri Java bindings.
 *
 * This test file is located in the unified tests directory (tests/Java/)
 * rather than in the source code directory, following the project's testing convention.
 */
public class TestAll {
    public static void main(String[] args) {
        System.out.println("=== Ri Java Binding Test ===\n");
        
        int passed = 0;
        int failed = 0;
        
        // Test RiAppBuilder
        try {
            RiAppBuilder builder = new RiAppBuilder();
            System.out.println("[PASS] RiAppBuilder created");
            builder.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiAppBuilder: " + e.getMessage());
            failed++;
        }
        
        // Test RiAppBuilder chaining (returns new instance)
        try {
            RiAppBuilder builder1 = new RiAppBuilder();
            RiAppBuilder builder2 = builder1.withConfig("config.yaml");
            
            if (builder1 != builder2) {
                System.out.println("[PASS] RiAppBuilder chaining creates new instance");
                passed++;
            } else {
                System.out.println("[FAIL] RiAppBuilder chaining should create new instance");
                failed++;
            }
            
            builder1.close();
            builder2.close();
        } catch (Exception e) {
            System.out.println("[FAIL] RiAppBuilder chaining: " + e.getMessage());
            failed++;
        }
        
        // Test RiConfig
        try {
            RiConfig config = new RiConfig();
            System.out.println("[PASS] RiConfig created");
            config.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiConfig: " + e.getMessage());
            failed++;
        }
        
        // Test RiCacheModule
        try {
            RiCacheConfig cacheConfig = new RiCacheConfig();
            RiCacheModule cache = new RiCacheModule(cacheConfig);
            System.out.println("[PASS] RiCacheModule created");
            cache.close();
            cacheConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiCacheModule: " + e.getMessage());
            failed++;
        }
        
        // Test RiValidationModule
        try {
            RiValidationResult result = RiValidationModule.validateEmail("test@example.com");
            System.out.println("[PASS] RiValidationModule.validateEmail: valid=" + result.isValid());
            result.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiValidationModule: " + e.getMessage());
            failed++;
        }
        
        // Test RiAuthModule
        try {
            RiAuthConfig authConfig = new RiAuthConfig();
            RiAuthModule auth = new RiAuthModule(authConfig);
            System.out.println("[PASS] RiAuthModule created");
            auth.close();
            authConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiAuthModule: " + e.getMessage());
            failed++;
        }
        
        // Test RiGateway
        try {
            RiGateway gateway = new RiGateway();
            System.out.println("[PASS] RiGateway created");
            gateway.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiGateway: " + e.getMessage());
            failed++;
        }
        
        // Test RiQueueModule
        try {
            RiQueueConfig queueConfig = new RiQueueConfig();
            RiQueueModule queue = new RiQueueModule(queueConfig);
            System.out.println("[PASS] RiQueueModule created");
            queue.close();
            queueConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiQueueModule: " + e.getMessage());
            failed++;
        }
        
        // Test RiDatabaseConfig
        try {
            RiDatabaseConfig dbConfig = new RiDatabaseConfig();
            System.out.println("[PASS] RiDatabaseConfig created");
            dbConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiDatabaseConfig: " + e.getMessage());
            failed++;
        }
        
        // Test RiServiceMesh
        try {
            RiServiceMeshConfig meshConfig = new RiServiceMeshConfig();
            RiServiceMesh mesh = new RiServiceMesh(meshConfig);
            System.out.println("[PASS] RiServiceMesh created");
            mesh.close();
            meshConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiServiceMesh: " + e.getMessage());
            failed++;
        }
        
        // Test RiObservabilityModule
        try {
            RiObservabilityConfig obsConfig = new RiObservabilityConfig();
            RiObservabilityModule obs = new RiObservabilityModule(obsConfig);
            System.out.println("[PASS] RiObservabilityModule created");
            obs.close();
            obsConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiObservabilityModule: " + e.getMessage());
            failed++;
        }
        
        // Test RiDeviceControlModule
        try {
            RiDeviceControlConfig deviceConfig = new RiDeviceControlConfig();
            RiDeviceControlModule device = new RiDeviceControlModule(deviceConfig);
            System.out.println("[PASS] RiDeviceControlModule created");
            device.close();
            deviceConfig.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiDeviceControlModule: " + e.getMessage());
            failed++;
        }
        
        // Test RiLogger
        try {
            RiLogger logger = new RiLogger();
            System.out.println("[PASS] RiLogger created");
            logger.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiLogger: " + e.getMessage());
            failed++;
        }
        
        // Test RiFileSystem
        try {
            RiFileSystem fs = new RiFileSystem();
            System.out.println("[PASS] RiFileSystem created");
            fs.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiFileSystem: " + e.getMessage());
            failed++;
        }
        
        // Test RiGrpcServer
        try {
            RiGrpcServer grpcServer = new RiGrpcServer();
            System.out.println("[PASS] RiGrpcServer created");
            grpcServer.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiGrpcServer: " + e.getMessage());
            failed++;
        }
        
        // Test RiGrpcClient
        try {
            RiGrpcClient grpcClient = new RiGrpcClient();
            System.out.println("[PASS] RiGrpcClient created");
            grpcClient.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiGrpcClient: " + e.getMessage());
            failed++;
        }
        
        // Test RiWSServer
        try {
            RiWSServer wsServer = new RiWSServer();
            System.out.println("[PASS] RiWSServer created");
            wsServer.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiWSServer: " + e.getMessage());
            failed++;
        }
        
        // Test RiWSClient
        try {
            RiWSClient wsClient = new RiWSClient();
            System.out.println("[PASS] RiWSClient created");
            wsClient.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiWSClient: " + e.getMessage());
            failed++;
        }
        
        // Test RiProtocolManager
        try {
            RiProtocolManager protocol = new RiProtocolManager();
            System.out.println("[PASS] RiProtocolManager created");
            protocol.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiProtocolManager: " + e.getMessage());
            failed++;
        }
        
        // Test RiHookBus
        try {
            RiHookBus hookBus = new RiHookBus();
            System.out.println("[PASS] RiHookBus created");
            hookBus.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiHookBus: " + e.getMessage());
            failed++;
        }
        
        // Test RiModuleRPC
        try {
            RiModuleRPC moduleRpc = new RiModuleRPC();
            System.out.println("[PASS] RiModuleRPC created");
            moduleRpc.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] RiModuleRPC: " + e.getMessage());
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
